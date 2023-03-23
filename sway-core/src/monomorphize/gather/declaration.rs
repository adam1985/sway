use sway_error::handler::{ErrorEmitted, Handler};

use crate::{decl_engine::DeclId, language::ty, monomorphize::priv_prelude::*, SubstList};

pub(crate) fn gather_from_decl(
    ctx: GatherContext,
    handler: &Handler,
    decl: &ty::TyDecl,
) -> Result<(), ErrorEmitted> {
    match decl {
        ty::TyDecl::VariableDecl(decl) => {
            gather_from_exp(ctx, handler, &decl.body)?;
        }
        ty::TyDecl::ConstantDecl { .. } => todo!(),
        ty::TyDecl::FunctionDecl {
            decl_id,
            subst_list,
            ..
        } => {
            gather_from_fn_decl(ctx, handler, decl_id, subst_list.inner())?;
        }
        ty::TyDecl::TraitDecl { .. } => todo!(),
        ty::TyDecl::StructDecl { .. } => todo!(),
        ty::TyDecl::EnumDecl { .. } => todo!(),
        ty::TyDecl::ImplTrait { .. } => todo!(),
        ty::TyDecl::AbiDecl { .. } => todo!(),
        ty::TyDecl::GenericTypeForFunctionScope { .. } => todo!(),
        ty::TyDecl::StorageDecl { .. } => todo!(),
        ty::TyDecl::ErrorRecovery(_) => {}
        ty::TyDecl::TypeAliasDecl { .. } => todo!(),
    }

    Ok(())
}

fn gather_from_fn_decl(
    mut ctx: GatherContext,
    handler: &Handler,
    decl_id: &DeclId<ty::TyFunctionDecl>,
    subst_list: &SubstList,
) -> Result<(), ErrorEmitted> {
    let decl = ctx.decl_engine.get_function(decl_id);

    if !subst_list.is_empty() {
        unimplemented!("{}", decl.name);
    }

    let ty::TyFunctionDecl {
        body,
        parameters,
        return_type,
        ..
    } = decl;

    parameters.iter().for_each(|param| {
        ctx.add_constraint(param.type_argument.type_id.into());
    });
    ctx.add_constraint(return_type.type_id.into());
    gather_from_code_block(ctx.by_ref(), handler, &body)?;

    Ok(())
}
